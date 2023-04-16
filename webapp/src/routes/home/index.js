import { Component, h } from 'preact';
import style from './style.css';

class Home extends Component {
	state = { states: {} };

	constructor() {
		super();

		this.setState({ states: {} });

		const socket = new WebSocket('ws://localhost:3000/api/ws');
		socket.addEventListener('open', function (event) {
			socket.send('Hello Server!');
		});

		socket.addEventListener('message', this.onSocketMessage.bind(this));

	}

	onSocketMessage(event) {
		const msg = JSON.parse(event.data);
		this.state.states[msg.mac] = msg;
		this.setState(this.state);
	}

	componentDidMount() {
		console.debug("componentDidMount");
	}

	componentWillUnmount() {
		console.debug("componentWillUnmount");
	}

	render(props, state) {
		return (
			<article class="app">
				<h1>Sensors</h1>
				<ul>
					{
						Object.values(state.states).map(msg => (
							<li key={msg}>{msg.mac} : {msg.temperature}</li>
						))
					}
				</ul>
			</article>
		);
	}
}


export default Home;
