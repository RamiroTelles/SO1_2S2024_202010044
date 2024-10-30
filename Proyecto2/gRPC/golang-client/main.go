package main

import (
	"context"
	"flag"
	"fmt"
	pb "golang-client/proto"
	"log"
	"time"

	"github.com/gofiber/fiber/v2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
	addr1 = flag.String("addr1", "grpc-server-service-swimming:50051", "the address to connect to")
	addr2 = flag.String("addr2", "grpc-server-service-running:50051", "the address to connect to")
	addr3 = flag.String("addr3", "grpc-server-service-boxing:50051", "the address to connect to")
)

type Student struct {
	Name       string `json:"name"`
	Age        int    `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int    `json:"discipline"`
}

func sendData(fiberCtx *fiber.Ctx) error {
	var body Student
	if err := fiberCtx.BodyParser(&body); err != nil {
		return fiberCtx.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	address := *addr1
	if body.Discipline == 1 {
		address = *addr2
	} else if body.Discipline == 2 {
		address = *addr3
	}
	fmt.Println(body)

	conn, err := grpc.NewClient(address, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewStudentClient(conn)

	// Create a channel to receive the response and error
	responseChan := make(chan *pb.StudentResponse)
	errorChan := make(chan error)

	go func() {

		// Contact the server and print out its response.
		ctx, cancel := context.WithTimeout(context.Background(), time.Second)
		defer cancel()

		r, err := c.SendStudent(ctx, &pb.StudentRequest{
			Name:       body.Name,
			Age:        int32(body.Age),
			Faculty:    body.Faculty,
			Discipline: pb.Discipline(body.Discipline),
		})

		if err != nil {
			errorChan <- err
			return
		}

		responseChan <- r
	}()

	select {
	case response := <-responseChan:
		return fiberCtx.JSON(fiber.Map{
			"message": response.GetSuccess(),
		})
	case err := <-errorChan:
		return fiberCtx.Status(500).JSON(fiber.Map{
			"error": err.Error(),
		})
	case <-time.After(5 * time.Second):
		return fiberCtx.Status(500).JSON(fiber.Map{
			"error": "timeout",
		})
	}
}

func main() {
	app := fiber.New()
	app.Post("/grpc-go", sendData)

	err := app.Listen(":8080")
	if err != nil {
		log.Println(err)
		return
	}
}
