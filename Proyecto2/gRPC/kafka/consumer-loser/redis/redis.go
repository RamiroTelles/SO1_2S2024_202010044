package redis

import (
	"consumer-winner/structs"
	"context"
	"fmt"
	"log"
	"sync"

	"github.com/redis/go-redis/v9"
)

var redisLock = &sync.Mutex{}

var redisClient *redis.Client

func Connect2Redis() *redis.Client {

	host := "redis"
	port := "6379"
	client := redis.NewClient(&redis.Options{
		Addr:     host + ":" + port,
		Password: "rami123",
		DB:       0,
	})

	return client
}

func GetRedisInstance() *redis.Client {

	if redisClient == nil {
		redisLock.Lock()
		defer redisLock.Unlock()
		if redisClient == nil {
			fmt.Println("Creating single redis instance now.")
			redisClient = Connect2Redis()
		} else {
			fmt.Println("Single instance already created.")
		}
	} else {
		fmt.Println("Single instance already created.")
	}

	return redisClient
}

func Insert(value structs.Student) {
	ctx := context.Background()
	client := GetRedisInstance()

	newValue, err := client.HIncrBy(ctx, value.Faculty, "lose", 1).Result()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Nuevo valor de %s en %s: %d\n", value.Faculty, "lose", newValue)

	newValue, err = client.HIncrBy(ctx, value.Faculty, "total", 1).Result()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Nuevo valor de %s en %s: %d\n", value.Faculty, "total", newValue)
}
