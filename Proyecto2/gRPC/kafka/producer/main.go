package main

import (
	"context"
	"encoding/json"
	"log"
	"time"

	"github.com/segmentio/kafka-go"
)

type Student struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func sendData(data Student) {
	topic := "winner"
	partition := 0

	conn, err := kafka.DialLeader(context.Background(), "tcp", "my-cluster-kafka-bootstrap:9092", topic, partition)
	if err != nil {
		log.Fatal("Error al Dial", err)
	}

	valueBytes, err := json.Marshal(data)
	if err != nil {
		log.Fatal("Error con json:", err)
	}

	conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
	_, err = conn.WriteMessages(
		kafka.Message{Value: valueBytes},
	)

	if err != nil {
		log.Fatal("Error al mandar el mensaje:", err)
	}

	if err := conn.Close(); err != nil {
		log.Fatal("Error al cerrar la conexion:", err)
	}

	log.Println("Mensaje enviado")
}

func main() {

	data := Student{"Rami kafka", 22, "ingenieria", 0}

	for {
		sendData(data)

		time.Sleep(30 * time.Second)
	}

}
