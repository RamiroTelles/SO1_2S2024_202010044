package kafka

import (
	"context"
	"encoding/json"
	"golang-server/structs"
	"log"
	"time"

	"github.com/segmentio/kafka-go"
)

func SendData(data structs.Student, topic string) {
	//topic := "winner"
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
