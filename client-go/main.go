package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"os/signal"
	"strings"
	"syscall"
)

func main() {
	fmt.Println("===========================================")
	fmt.Println("       TCP Chat Client - Go")
	fmt.Println("===========================================")

	serverAddr := "localhost:8080"
	if len(os.Args) > 1 {
		serverAddr = os.Args[1]
	}

	fmt.Print("Digite seu nome de usuario: ")
	reader := bufio.NewReader(os.Stdin)
	username, _ := reader.ReadString('\n')
	username = strings.TrimSpace(username)

	if username == "" {
		username = "Anonymus"
	}

	conn, err := net.Dial("tcp", serverAddr)
	if err != nil {
		fmt.Printf("Erro ao conectar ao servidor: %v\n", err)
		os.Exit(1)
	}
	defer conn.Close()

	fmt.Fprintf(conn, "%s\n", username)

	response := make([]byte, 1024)
	n, err := conn.Read(response)
	if err != nil {
		fmt.Println("Erro ao ler resposta do servidor")
		os.Exit(1)
	}

	responseMsg := string(response[:n])
	if strings.Contains(responseMsg, "Erro") {
		fmt.Print(responseMsg)
		os.Exit(1)
	}

	fmt.Println("Conectado ao servidor! Digite /quit para sair.")
	fmt.Println("-------------------------------------------")

	signalChan := make(chan os.Signal, 1)
	signal.Notify(signalChan, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		<-signalChan
		fmt.Println("\nSaindo...")
		fmt.Fprintf(conn, "/quit\n")
		os.Exit(0)
	}()

	go func() {
		scanner := bufio.NewScanner(conn)
		for scanner.Scan() {
			msg := scanner.Text()
			if msg != "" {
				fmt.Printf("\r%s\n> ", msg)
			}
		}
	}()

	for {
		fmt.Print("> ")
		text, _ := reader.ReadString('\n')
		text = strings.TrimSpace(text)

		if text == "" {
			continue
		}

		if text == "/quit" {
			fmt.Fprintf(conn, "/quit\n")
			fmt.Println("Desconectado.")
			break
		}

		_, err := fmt.Fprintf(conn, "%s\n", text)
		if err != nil {
			fmt.Printf("Erro ao enviar mensagem: %v\n", err)
			break
		}
	}
}
