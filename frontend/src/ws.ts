import type { SensorReading } from "./proto/temperature";

export default function initWS(){
  const socket = new WebSocket("ws://localhost:3000/ws");

  socket.onopen = () => console.log("✅ WebSocket connected");
  socket.onclose = () => console.log("❌ WebSocket disconnected");
  socket.onerror = (e) => console.log("🚨 Error:", e);

  socket.onmessage = (event) => {
    try {
      const data: SensorReading = JSON.parse(event.data);
      console.log(data);
    } catch {
      console.log("Raw data:", event.data);
    }
  };

  return socket;
}