#include <Arduino.h>

#include "MeOrion.h"

enum Order {
  HELLO = 0,
  CRANE_DIRECTION = 1,
  MOTOR = 2,
  GRABBER = 3,
  ALREADY_CONNECTED = 4,
  ERROR = 5,
  RECEIVED = 6,
  STOP = 7,
  SYNC_CONFIG = 8,
  CLOSE = 9,
  ULTRASONIC = 10
};

MeDCMotor grabber(PORT_1);

MeDCMotor crane(PORT_2);

MeDCMotor right_motor(M1);

MeDCMotor left_motor(M2);

MeUltrasonicSensor ultraSensor(PORT_4);

bool is_connected = false; ///< True if the connection with the master is available

int motor_left_speed = 0;
int motor_right_speed = 0;

int crane_direction = 0;
int grabber_direction = 0;

int crane_speed = 70;
int grabber_speed = 100;

int debug = 0;

void setup()
{
  // Init Serial
  Serial.begin(115200);

  // Wait until the arduino is connected to master
  while(!is_connected)
  {
    write_order(HELLO);
    wait_for_bytes(1, 1000);
    get_messages_from_serial();
  }

}

void loop()
{ 
  get_messages_from_serial();
  update_orders();
}

void update_orders()
{
  setCrane();
  setGrabber();
  setMotor();
}

void setMotor() {
  left_motor.run(motor_left_speed * -1);
  right_motor.run(motor_right_speed * -1);
}

void setCrane() {
  crane.run(crane_speed * crane_direction);
}

void setGrabber() {
  grabber.run(grabber_speed * grabber_direction);
}

void stop()
{
  crane.stop();
  grabber.stop();
  left_motor.stop();
  right_motor.stop();
}

void get_messages_from_serial()
{
  if(Serial.available() > 0)
  {
    // The first byte received is the instruction
    Order order_received = read_order();

    if(order_received == HELLO)
    {
      // If the cards haven't say hello, check the connection
      if(!is_connected)
      {
        is_connected = true;
        write_order(HELLO);
      }
      else
      {
        // If we are already connected do not send "hello" to avoid infinite loop
        write_order(ALREADY_CONNECTED);
      }
    }
    else if(order_received == ALREADY_CONNECTED)
    {
      is_connected = true;
    }
    else
    {
      write_order(RECEIVED);
      switch(order_received)
      {
        case STOP:
        {
          stop();
          if (debug) {
              write_order(STOP);
          }
          break;
        }
        case MOTOR:
        {
          motor_left_speed = read_i16();
          motor_right_speed = read_i16();
          if (debug) {
              write_order(MOTOR);
              write_i16(motor_left_speed);
              write_i16(motor_right_speed);
          }
          break;
        }
        case GRABBER:
        {
          grabber_direction = read_i8();
          if (debug) {
              write_order(GRABBER);
              write_i8(grabber_direction);
          }
          break;
        }
        case CRANE_DIRECTION:
        {
          crane_direction = read_i8();
          if (debug) {
              write_order(CRANE_DIRECTION);
              write_i8(crane_direction);
          }
          break;
        }
        case CLOSE:
        {
            is_connected = false;
            stop();
            if (debug) {
              write_order(CLOSE);
            }
            break;
        }
        case SYNC_CONFIG:
        {
            debug = read_i8();
            if (debug) {
                write_order(SYNC_CONFIG);
                write_i8(debug);
            }
            break;
        }
        case ULTRASONIC:
        {
            if (debug) {
                write_order(ULTRASONIC);
            }
            write_i32((int32_t) (ultraSensor.distanceCm() * 100));
            break;
        }
       // Unknown order
        default:
          write_order(ERROR);
          write_i16(404);
          return;
      }
      
    }
    
  }
}


Order read_order()
{
  return (Order) Serial.read();
}

void wait_for_bytes(int num_bytes, unsigned long timeout)
{
  unsigned long startTime = millis();
  //Wait for incoming bytes or exit if timeout
  while ((Serial.available() < num_bytes) && (millis() - startTime < timeout)){}
}

// NOTE : Serial.readBytes is SLOW
// this one is much faster, but has no timeout
void read_signed_bytes(int8_t* buffer, size_t n)
{
  size_t i = 0;
  int c;
  while (i < n)
  {
    c = Serial.read();
    if (c < 0) break;
    *buffer++ = (int8_t) c; // buffer[i] = (int8_t)c;
    i++;
  }
}

int8_t read_i8()
{
  wait_for_bytes(1, 100); // Wait for 1 byte with a timeout of 100 ms
  return (int8_t) Serial.read();
}

int16_t read_i16()
{
  int8_t buffer[2];
  wait_for_bytes(2, 100); // Wait for 2 bytes with a timeout of 100 ms
  read_signed_bytes(buffer, 2);
  return (((int16_t) buffer[0]) & 0xff) | (((int16_t) buffer[1]) << 8 & 0xff00);
}

int32_t read_i32()
{
  int8_t buffer[4];
  wait_for_bytes(4, 200); // Wait for 4 bytes with a timeout of 200 ms
  read_signed_bytes(buffer, 4);
  return (((int32_t) buffer[0]) & 0xff) | (((int32_t) buffer[1]) << 8 & 0xff00) | (((int32_t) buffer[2]) << 16 & 0xff0000) | (((int32_t) buffer[3]) << 24 & 0xff000000);
}

void write_order(enum Order myOrder)
{
  uint8_t* Order = (uint8_t*) &myOrder;
  Serial.write(Order, sizeof(uint8_t));
}

void write_i8(int8_t num)
{
  Serial.write(num);
}

void write_i16(int16_t num)
{
  int8_t buffer[2] = {(int8_t) (num & 0xff), (int8_t) (num >> 8)};
  Serial.write((uint8_t*)&buffer, 2*sizeof(int8_t));
}

void write_i32(int32_t num)
{
  int8_t buffer[4] = {(int8_t) (num & 0xff), (int8_t) (num >> 8 & 0xff), (int8_t) (num >> 16 & 0xff), (int8_t) (num >> 24 & 0xff)};
  Serial.write((uint8_t*)&buffer, 4*sizeof(int8_t));
}
