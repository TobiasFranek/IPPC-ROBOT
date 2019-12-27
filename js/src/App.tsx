import React, { useEffect, useState } from 'react';
import { library } from '@fortawesome/fontawesome-svg-core';
import { faCompressAlt, faArrowsAltH, faLongArrowAltDown, faLongArrowAltUp } from '@fortawesome/free-solid-svg-icons';
import Grabber from './components/Grabber/Grabber';
import Crane from './components/Crane/Crane';
import Steering from './components/Steering/Steering';

library.add(faCompressAlt, faArrowsAltH, faLongArrowAltDown, faLongArrowAltUp)

interface AppProps {
	socket: WebSocket
}

export default ({ socket } : AppProps) => {

	const [ ultrasonic, setUltrasonic ] = useState<number>(0);

	useEffect(() => {
		socket.onmessage = (e) => {
			const data = JSON.parse(e.data);
			if (data.event === 'ULTRASONIC') {
				setUltrasonic(data.config.value/100);
			}
		}
		polling();
	}, []);

	const polling = () => {
		const event = {
			event: 'ULTRASONIC'
		}
		socket.send(JSON.stringify(event));
		return setTimeout(polling, 1000);
	}


	const onGrabberChange = (direction: number) => {
		const event = {
			event: 'GRABBER',
			config: {
				direction
			}
		}
		socket.send(JSON.stringify(event));
	}

	const onCraneChange = (direction: number) => {
		const event = {
			event: 'CRANE_DIRECTION',
			config: {
				direction
			}
		}
		socket.send(JSON.stringify(event));
	}

	const onSteeringChange = (params : { left: number, right: number }) => {
		const event = {
			event: 'MOTOR',
			config: params
		
		}
		socket.send(JSON.stringify(event));
	}

	return (
		<div className="app"> 
			<div className="ultrasonic-font">{ultrasonic} cm</div>
			<Grabber onChange={onGrabberChange}/>
			<Crane onChange={onCraneChange} />
			<Steering onChange={onSteeringChange}/>
		</div>
	);
}