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

	discipline := "swimming"

	if value.Discipline == 1 {
		discipline = "running"
	} else if value.Discipline == 2 {
		discipline = "boxing"
	}

	newValue, err := client.HIncrBy(ctx, value.Faculty, discipline, 1).Result()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Nuevo valor de %s en %s: %d\n", value.Faculty, discipline, newValue)

	newValue, err = client.HIncrBy(ctx, value.Faculty, "total", 1).Result()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Nuevo valor de %s en %s: %d\n", value.Faculty, "total", newValue)

}

// keyFacultyDiscipline := fmt.Sprintf("%s:%d", value.Faculty, value.Discipline)
// keyFacultyTotal := fmt.Sprintf("%s:total", value.Faculty)

// // create or get if exists the key
// //counter1 := int(client.Incr(context.Background(), keyFacultyDiscipline).Val())

// // insert the name as the field and the value as the counter in the "counter" hash
// err := client.HSetInc(context.TODO(), "counter", keyFacultyDiscipline, counter1).Err()
// if err != nil {
// 	log.Println("Error saving on redis: ", err)
// }

// //counter2 := int(client.Incr(context.Background(), keyFacultyTotal).Val())

// // insert the name as the field and the value as the counter in the "counter" hash
// err = client.HSet(context.TODO(), "counter", keyFacultyDiscipline, counter2).Err()
// if err != nil {
// 	log.Println("Error saving on redis: ", err)
// }

// log.Println("value saved on redis -> ", value)
