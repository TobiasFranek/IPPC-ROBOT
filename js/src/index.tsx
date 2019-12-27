import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './assets/scss/globals.scss';
import * as serviceWorker from './serviceWorker';

if (process.env.REACT_APP_WEBSOCKET) {
	const socket = new WebSocket(process.env.REACT_APP_WEBSOCKET, 'rust-websocket');

	socket.onopen = () => {
		console.log('Connected to websocket.');
		ReactDOM.render(<App socket={socket}/>, document.getElementById('root'));
	};

	socket.onerror = () => {
		const errorContainer = document.querySelector('.error-container');
		if (errorContainer) {
			errorContainer.innerHTML = '<div class="error">Something went wrong with the Connection.</div>'
		}
	}
}




// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
