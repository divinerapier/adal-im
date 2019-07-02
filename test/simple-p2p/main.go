package main

import (
	"encoding/binary"
	"encoding/json"
	"fmt"
	"io"
	"math/rand"
	"net"
	"time"
)

func main() {
	go setup(1, 2)
	setup(2, 1)
}

func setup(from, to uint64) {
	conn, err := net.Dial("tcp", "127.0.0.1:6810")
	if err != nil {
		panic(err)
	}
	go readLoop(from, conn)
	login(from, conn)
	for {
		sendPrivateTextMessage(from, to, conn)
	}
}

func login(user uint64, conn net.Conn) {
	data := NewPacket(user, 0, []byte{}).Encode()
	conn.Write(data)
}

type TextMessage struct {
	From    uint64 `json:"from,omitempty"`
	To      uint64 `json:"to,omitempty"`
	Message string `json:"message,omitempty"`
}

func sendPrivateTextMessage(from, to uint64, conn net.Conn) {
	tm := TextMessage{
		From:    from,
		To:      to,
		Message: fmt.Sprintf("I'm user: %d. send at %d", from, time.Now().Unix()),
	}
	data, err := json.Marshal(&tm)
	if err != nil {
		panic(err)
	}
	data = NewPacket(from, 1000, data).Encode()
	// fmt.Println(data)
	n, err := conn.Write(data)
	if err != nil {
		panic(err)
	}
	if n != len(data) {
		fmt.Printf("n = %d, len(data) = %d\n", n, len(data))
	}
	time.Sleep(time.Microsecond * time.Duration(rand.Int()%1000))
}

func readLoop(user uint64, conn net.Conn) {
	for {
		packet, err := readPacket(conn)
		if err != nil {
			if err == io.EOF {
				return
			}
			fmt.Println("read packet with error:", err)
		}
		fmt.Printf("user: %d, packet message: %s\n", user, packet.Message)
	}
}

func readPacket(conn net.Conn) (Packet, error) {
	var header [4]byte
	_, err := io.ReadFull(conn, header[:])
	if err != nil {
		return Packet{}, err
	}
	length := binary.LittleEndian.Uint32(header[:])
	body := make([]byte, length)
	n, err := io.ReadFull(conn, body)
	if err != nil {
		return Packet{}, err
	}
	if n == 0 {
		return Packet{}, nil
	}
	if n != int(length) {
		panic(fmt.Sprintf("length: %d, n: %d\n", length, n))
	}
	p := Packet{
		TotalLength: binary.LittleEndian.Uint32(body[:4]),
		RequestUser: binary.LittleEndian.Uint64(body[4:12]),
		MessageType: binary.LittleEndian.Uint32(body[12:16]),
		Message:     body[24:],
	}
	messageLength := binary.LittleEndian.Uint32(body[16:24])
	if int(messageLength) != len(p.Message) {
		panic(fmt.Sprintf("expect length: %d, actual length: %d", messageLength, len(p.Message)))
	}
	return p, nil
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
