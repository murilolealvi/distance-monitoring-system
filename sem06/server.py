import socket
import threading

PORT = 5050
#SERVER = "localhost"
SERVER = socket.gethostbyname(socket.gethostname())
ADDR = (SERVER, PORT)
HEADER = 64 #64 bits enviados por mensagem
FORMAT = 'utf-8'
DISCONNECTED = 'Lost connection...'


server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.bind(ADDR)

def handle_client(conn, addr):
    print(addr, 'connected')
    connected = True
    while connected:
        msg_len = conn.recv(HEADER).decode(FORMAT)
        if msg_len:
            msg_len = int(msg_len)
            msg = conn.recv(msg_len).decode(FORMAT)
            if msg == DISCONNECTED:
                connected = False
            print(addr, msg)
            conn.send('Message received!'.encode(FORMAT))
    conn.close()
def start():
    server.listen()
    print('Listening on', SERVER)
    while True:
        conn, addr = server.accept()
        t = threading.Thread(target=handle_client, args=(conn,addr))
        t.start()
        print('Active connections: ', threading.active_count()-1)

print('Server is up...')
start()