import { useState, useEffect } from 'react'

import './App.css'
import initWS from './ws'
import type { SensorReading } from './proto/temperature';
import TempChart from './components/TempChart';

function App() {
  const [readings, setReadings] = useState<SensorReading[]>([]);
  useEffect(() => {
    const ws = initWS();
    ws.onmessage = (event) => {
      const reading: SensorReading = JSON.parse(event.data);
      setReadings((prev) => [...prev.slice(-49), reading]);
    };

    return () => {
      if (ws.readyState === 1) {
        ws.close();
      } else {
        ws.addEventListener('open', () => {
          ws.close();
        });
      }
    }
  }, []);

  if(readings.length < 1) {
    return <> Loading data...</>
  } 
    return <div>
      <div>{readings[readings.length - 1].temperature}°F</div>
      <div>{new Date(readings[readings.length - 1].timestamp * 1000).toLocaleTimeString()}</div>
      <TempChart readings={readings} />
    </div>
}

export default App
