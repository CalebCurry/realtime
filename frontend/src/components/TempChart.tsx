import { Line } from "react-chartjs-2";
import {
  Chart as ChartJS,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement,
  Tooltip,
} from "chart.js";

ChartJS.register(LineElement, CategoryScale, LinearScale, PointElement, Tooltip);

export default function TempChart({ readings }: { readings: { timestamp: number; temperature: number }[] }) {
  const data = {
    labels: readings.map(r =>
      new Date(r.timestamp * 1000).toLocaleTimeString()
    ),
    datasets: [
      {
        label: "Temperature (°F)",
        data: readings.map(r => r.temperature),
        borderColor: "#36a2eb",
        tension: 0.3,
      },
    ],
  };

  return <Line data={data} options={{ responsive: true, plugins: { legend: { display: false } } }} />;
}
