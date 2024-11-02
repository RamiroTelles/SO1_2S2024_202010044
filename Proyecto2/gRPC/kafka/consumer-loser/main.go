package main

import (
	"consumer-winner/redis"
	"consumer-winner/structs"
	"context"
	"encoding/json"
	"fmt"
	"log"

	"github.com/google/uuid"
	"github.com/segmentio/kafka-go"
)

func main() {

	topic := "loser"

	r := kafka.NewReader(kafka.ReaderConfig{
		Brokers: []string{"my-cluster-kafka-bootstrap:9092"},
		Topic:   topic,

		MaxBytes:    10e6,
		StartOffset: kafka.LastOffset,
		GroupID:     uuid.New().String(),
	})

	for {
		fmt.Println("Conectandose...")
		fmt.Println(topic)
		m, err := r.ReadMessage(context.Background())
		if err != nil {
			log.Println("Error al leer", err)
			break

		}
		fmt.Printf("mensaje %d: %s= %s\n", m.Offset, string(m.Key), string(m.Value))

		//mandar a redis
		fmt.Println("Mandar a redis")
		RedisInset(m.Value)

		err = r.CommitMessages(context.Background(), m)
		if err != nil {
			log.Println("Error al commit", err)
		}
	}

	if err := r.Close(); err != nil {
		log.Fatal("Error al cerrar lector")
	}
}

func RedisInset(data []byte) {

	var jsonData structs.Student
	err := json.Unmarshal(data, &jsonData)
	if err != nil {
		fmt.Printf("Failed to unmarshal message: %s", err)
		return
	}

	go redis.Insert(jsonData)
}
