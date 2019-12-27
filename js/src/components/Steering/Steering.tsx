import React, { useState, useEffect } from 'react';
import ReactNipple from 'react-nipple';

interface SteeringProps {
	onChange?: Function
}

export default ({ onChange }: SteeringProps) => {
	const [ left, setLeft ] = useState<number>(0);
	const [ right, setRight ] = useState<number>(0);
	const [ oldLeft, setOldLeft ] = useState<number>(left);
	const [ oldRight, setOldRight ] = useState<number>(right);

	useEffect(() => {
		if (onChange && (left !== oldLeft || right !== oldRight)) {
			onChange({
				left,
				right
			});

			setOldLeft(left);
			setOldRight(right);
		}
	}, [left, right])

	const mapNumber = (number: number, in_min: number, in_max: number, out_min: number, out_max: number) => {
		return (number - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
	}

	const clamp = (number: number, min: number, max: number) => {
		return Math.min(Math.max(number, min), max);
	}

	const onMove = (evt: any, data: any) => {
		let x = data.distance * Math.cos(data.angle.radian);
		let y = data.distance * Math.sin(data.angle.radian);

		let speedLeft = mapNumber(y, -50, 50, -255, 255);
		let speedRight = mapNumber(y, -50, 50, -255, 255);

		let diffSpeed = mapNumber(x, -50, 50, -255, 255);
		if (x < 0) {
			speedLeft -= diffSpeed;
			speedRight += diffSpeed;
		} else {
			speedLeft += diffSpeed;
			speedRight -= diffSpeed;
		}

		speedLeft = clamp(speedLeft, -255, 255);
		speedRight = clamp(speedRight, -255, 255);

		if (speedLeft >= -10 && speedLeft <= 10) {
			speedLeft = 0;
		}
		if (speedRight >= -10 && speedRight <= 10) {
			speedRight = 0;
		}

		setLeft(Math.floor(speedLeft));
		setRight(Math.floor(speedRight));
	}

	const reset = () => {
		setLeft(0);
		setRight(0);
	}

	return (
		<ReactNipple
			// supports all nipplejs options
			// see https://github.com/yoannmoinet/nipplejs#options
			options={{ mode: 'dynamic', position: { top: '50%', left: '50%' } }}
			// any unknown props will be passed to the container element, e.g. 'title', 'style' etc
			style={{
				width: '100vw',
				height: '100vh'
				// if you pass position: 'relative', you don't need to import the stylesheet
			}}
			// all events supported by nipplejs are available as callbacks
			// see https://github.com/yoannmoinet/nipplejs#start
			onMove={onMove}
			onEnd={reset}
		/>
	)
}