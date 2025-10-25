package main

import (
	"bytes"
	"fmt"
	"math/rand"
	"net/http"
	"time"

	"google.golang.org/protobuf/proto"

	temperature "sensors/proto"
)

func main() {
	for {
		go func() {
			data, err := proto.Marshal(&temperature.SensorReading{
				SensorId:    "abc123",
				Temperature: float64(72 + rand.Intn(5)),
				Timestamp:   time.Now().Unix(),
				Location:    "Living Room",
			})
			if err != nil {
				fmt.Println("Error marshaling proto:", err)
				return
			}

			resp, err := http.Post(
				"http://localhost:3000/api/sensor",
				"application/x-protobuf", // MIME type for protobuf data
				bytes.NewReader(data),
			)
			if err != nil {
				fmt.Println("Error sending POST:", err)
				return
			}
			defer resp.Body.Close()

			fmt.Println("Response:", resp.Status)
		}()

		time.Sleep(2 * time.Second)
	}
}
