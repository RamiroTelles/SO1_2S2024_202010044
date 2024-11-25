package main

import (
	"context"
	"flag"
	"fmt"
	"golang-server/kafka"
	"golang-server/structs"
	"log"
	"math/rand"
	"net"

	pb "golang-server/proto"

	"google.golang.org/grpc"
)

var (
	port = flag.Int("port", 50051, "The server port")
)

type server struct {
	pb.UnimplementedStudentServer
}

func (s *server) SendStudent(_ context.Context, in *pb.StudentRequest) (*pb.StudentResponse, error) {
	log.Printf("Received: %v", in)
	log.Printf("Student name: %s", in.GetName())
	log.Printf("Student faculty: %s", in.GetFaculty())
	log.Printf("Student age: %d", in.GetAge())
	log.Printf("Student discipline: %d", in.GetDiscipline())

	// Generamos un número aleatorio, 0 o 1
	result := rand.Intn(2)
	topic := "winner"
	if result == 1 {
		topic = "loser"
	}
	fmt.Println("topico enviado:", topic)
	go kafka.SendData(structs.Student{in.GetName(), int(in.GetAge()), in.GetFaculty(), int(in.GetDiscipline())}, topic)

	return &pb.StudentResponse{
		Success: true,
	}, nil
}

func main() {
	flag.Parse()
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterStudentServer(s, &server{})
	log.Printf("Server started on port %d", *port)

	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
