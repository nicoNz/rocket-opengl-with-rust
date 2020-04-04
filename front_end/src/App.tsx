import React from 'react';
import './App.css';


function sendData(v: number) {
  fetch('http://localhost:8000/' + v )
}

function App() {
  const [value, setValue] = React.useState(10)
  return (
    <div className="App">
      <label>
        intensity of our triangle
      </label>
      <input
        type="range"
        min="0"
        max="400"
        value={value}
        onChange={(e)=>{
          let v = parseFloat(e.target.value)
          setValue( v)
          sendData(v)
        }}
      />
    </div>
  );
}

export default App;
