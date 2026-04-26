# TCP Chat - Servidor Rust + Cliente Go

Sistema de chat em tempo real via terminal (CLI) com comunicação TCP entre múltiplos clientes.

## Arquitetura

```
[Go CLI Client] ---- TCP ----> [Rust CLI Server] ---- Broadcast ----> [Todos Clients]
```

- **Servidor**: Rust (alta performance)
- **Cliente**: Go (leve e rápido)
- **Comunicação**: TCP socket, texto puro

## Estrutura do Projeto

```
ChatTCP/
├── server-rust/
│   ├── Cargo.toml
│   └── src/main.rs
├── client-go/
│   └── main.go
└── README.md
```

---

## Como Compilar e Rodar

### Servidor (Rust)

1. Acesse o diretório do servidor:
   ```bash
   cd server-rust
   ```

2. Compile o servidor:
   ```bash
   cargo build --release
   ```

3. Execute o servidor:
   ```bash
   cargo run --release
   ```

O servidor iniciara na porta `8080` (0.0.0.0:8080).

   Ou use o binario compilado:
   ```bash
   ./target/release/server
   ```

---

### Cliente (Go)

1. Acesse o diretório do cliente:
   ```bash
   cd client-go
   ```

2. Execute o cliente:
   ```bash
   go run main.go
   ```

   Ou especifique um endereco diferente:
   ```bash
   go run main.go localhost:9090
   ```

3. Digite seu nome de usuario quando solicitado.

   Ou use o binario compilado:
   ```bash
   ./client
   ```

---

## Exemplos de Uso

### Terminal 1 - Servidor
```bash
$ cargo run --release
[2024-01-15T10:30:00] INFO: Servidor ouvindo em 0.0.0.0:8080
[2024-01-15T10:30:05] INFO: Novo cliente conectado: rafael (127.0.0.1:54321)
[2024-01-15T10:30:05] INFO: rafael: ola pessoal
[2024-01-15T10:31:00] INFO: Cliente 'rafael' desconectado
```

### Terminal 2 - Cliente 1
```
Digite seu nome de usuario: rafael
Conectado ao servidor!
> ola pessoal
> /quit
```

### Terminal 3 - Cliente 2
```
Digite seu nome de usuario: maria
Conectado ao servidor!
> rafael entrou no chat
> rafael: ola pessoal
>Oi rafael!
```

---

## Fluxo TCP

1. **Conexao**: Cliente conecta via TCP ao servidor
2. **Autenticacao**: Cliente envia nome de usuario
3. **Mensagens**: Cliente envia mensagens no formato `username: message`
4. **Broadcast**: Servidor redistribui mensagens para todos os clientes
5. **Desconexao**: Cliente envia `/quit` ou desconecta (SIGINT)

### Formato de Mensagem

```
username: message
```

Cada mensagem termina com `\n` (newline).

---

## Comandos

| Comando | Descricao |
|---------|-----------|
| `/quit` | Sair do chat |
| Ctrl+C | Desconectar |

---

## Tecnologias Usadas

- **Servidor**: Rust com `std::net`, threads, `Arc<Mutex>`
- **Cliente**: Go com `net`, `bufio`, goroutines
- **Protocolo**: TCP socket, texto puro

---

## Como Abrir Multiplos Clientes

1. Abra varios terminais
2. Em cada terminal, execute:
   ```bash
   cd client-go && go run main.go
   ```
3. Cada terminal sera um cliente independente

---

## Requisitos

- Rust (cargo) - para o servidor
- Go 1.x - para o cliente

---

## Licenca

MIT License