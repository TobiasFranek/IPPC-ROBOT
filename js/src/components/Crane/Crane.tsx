import React, { useState, useEffect } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import './Crane.scss';

interface CraneProps {
	onChange?: Function
}

export default ({ onChange } : CraneProps) => {

	const [ direction, setDirection ] = useState<number>(0);
	const [ oldDirection, setOldDirection ] = useState<number>(direction);

	useEffect(() => {
		if (onChange && direction !== oldDirection) {
			onChange(direction);
			setOldDirection(direction);
		}
	}, [direction])

	return (
		<div className="crane-controls grabber-controls">
			<h2 className="grabber-controls__headline">Crane Controls</h2>
			<div className="grabber-controls__wrapper">
				<button 
					onTouchStart={() => setDirection(-1)} 
					onMouseDown={() => setDirection(-1)} 
					onTouchEnd={() => setDirection(0)}
					onMouseUp={() => setDirection(0)} 
					className="grabber-controls__button crane-controls__button">
					<FontAwesomeIcon size="2x" icon={['fas', 'long-arrow-alt-up']}></FontAwesomeIcon>
				</button>
				<button 
					onTouchStart={() => setDirection(1)} 
					onMouseDown={() => setDirection(1)}
					onTouchEnd={() => setDirection(0)}
					onMouseUp={() => setDirection(0)} 
					className="grabber-controls__button">
					<FontAwesomeIcon size="2x" icon={['fas', 'long-arrow-alt-down']}></FontAwesomeIcon>
				</button>
			</div>
		</div>
	)
}