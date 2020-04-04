import React from 'react';
import './App.css';


function sendData(v: number) {
	fetch('http://localhost:8000/' + v)
}

function App() {
	const [value, setValue] = React.useState(10)
	const [camX, setCamX] = React.useState(10)
	const [camY, setCamY] = React.useState(10)
	return (
		<div className="App">

			<label>
				intensity of our triangle {value}
			</label>
			<input
				type="range"
				min="0"
				max="400"
				value={value}
				onChange={(e) => {
					let v = parseFloat(e.target.value)
					setValue(v)
					fetch('http://localhost:8000/model/x/' + v)
				}}
			/>


			<label>
				cameraX {camX}
			</label>
			<input
				type="range"
				min="-10"
				max="20"
				step=".01"
				value={camX}
				onChange={(e) => {
					let v = parseFloat(e.target.value)
					setCamX(v)
					fetch('http://localhost:8000/camera/x/' + v)
				}}
			/>

			<label>
				cameraY {camY}
			</label>
				<input
					type="range"
					min="-10"
					max="20"
					step=".01"
					value={camY}
					onChange={(e) => {
						let v = parseFloat(e.target.value)
						setCamY(v)
						fetch('http://localhost:8000/camera/y/' + v)
					}}
				/>
			</div>
		);
}

export default App;
