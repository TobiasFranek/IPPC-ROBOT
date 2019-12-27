import React, { useState, useEffect } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import './Grabber.scss';

interface GrabberProps {
	onChange?: Function
}

export default ({ onChange } : GrabberProps) => {

	const [ direction, setDirection ] = useState<number>(0);
	const [ oldDirection, setOldDirection ] = useState<number>(direction);

	useEffect(() => {
		if (onChange && direction !== oldDirection) {
			onChange(direction);
			setOldDirection(direction);
		}
	}, [direction])

	return (
		<div className="grabber-controls">
			<h2 className="grabber-controls__headline">Grabber Controls</h2>
			<div className="grabber-controls__wrapper">
				<button 
					onTouchStart={() => setDirection(1)} 
					onMouseDown={() => setDirection(1)} 
					onTouchEnd={() => setDirection(0)}
					onMouseUp={() => setDirection(0)} 
					className="grabber-controls__button">
					<FontAwesomeIcon size="2x" icon={['fas', 'compress-alt']}></FontAwesomeIcon>
				</button>
				<button 
					onTouchStart={() => setDirection(-1)} 
					onMouseDown={() => setDirection(-1)}
					onTouchEnd={() => setDirection(0)}
					onMouseUp={() => setDirection(0)} 
					className="grabber-controls__button">
					<FontAwesomeIcon size="2x" icon={['fas', 'arrows-alt-h']}></FontAwesomeIcon>
				</button>
			</div>
		</div>
	)
}