package main

import (
	"encoding/binary"
	"fmt"
	"math/rand"
	"net"
	"time"
)

func main() {
	go foo([]byte("client1"), 2)
	foo([]byte("client2"), 1002)
}

func foo(input []byte, mt uint32) {
	conn, err := net.Dial("tcp", "127.0.0.1:6810")
	if err != nil {
		panic(err)
	}
	for {
		data := NewPacket(1, mt, input).Encode()
		fmt.Printf("send. %v\n", data)
		n, err := conn.Write(data)
		if err != nil {
			panic(err)
		}
		if n != len(data) {
			fmt.Printf("n = %d, len(data) = %d\n", n, len(data))
		}
		var resp [1024]byte
		n, err = conn.Read(resp[:])
		if err != nil {
			panic(err)
		}
		fmt.Printf("resp: %s\n", resp[:n])
		time.Sleep(time.Microsecond * time.Duration(rand.Int()%1000))
	}
}

type Packet struct {
	TotalLength uint32
	RequestUser uint64
	MessageType uint32
	Message     []byte
}

func NewPacket(user uint64, mt uint32, message []byte) Packet {
	return Packet{
		TotalLength: 4 + 8 + 4 + 8 + uint32(len(message)),
		RequestUser: user,
		MessageType: mt,
		Message:     message,
	}
}

func (p Packet) Encode() []byte {
	length := p.TotalLength + 4
	data := make([]byte, length)
	binary.LittleEndian.PutUint32(data[:4], uint32(p.TotalLength))
	binary.LittleEndian.PutUint32(data[4:8], uint32(p.TotalLength))
	binary.LittleEndian.PutUint64(data[8:16], p.RequestUser)
	binary.LittleEndian.PutUint32(data[16:20], p.MessageType)
	binary.LittleEndian.PutUint32(data[20:28], uint32(len(p.Message)))
	copy(data[28:], p.Message)
	return data
}
