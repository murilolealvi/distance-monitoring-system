import socket


HEADER = 64
PORT = 5050
FORMAT = 'utf-8'
DISCONNECT = 'Lost connection...'
SERVER = socket.gethostbyname(socket.gethostname())
ADDR = (SERVER, PORT)

client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(ADDR)


def send(load):
    msg = load.encode(FORMAT)
    msg_len = len(msg)
    send_len = str(msg_len).encode(FORMAT)
    send_len += b' ' * (HEADER - len(send_len))
    client.send(send_len)
    client.send(msg)
    print(client.recv(1024).decode(FORMAT))

send('Testing server...')