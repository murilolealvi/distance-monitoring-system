import socket

TCP_IP = "127.0.0.1" #defining server address
FILE_PORT = 5005 #defining file port to transfer
DATA_PORT = 5006 #defining data port to transfer
buf = 1024 #defining the header size which will be send


sock_f = socket.socket(socket.AF_INET, socket.SOCK_STREAM) #initializes socket
sock_f.bind((TCP_IP, FILE_PORT)) #bind ip and port to the socket to get requests
sock_f.listen(1) #starting listening by limit 1 connection

sock_d = socket.socket(socket.AF_INET, socket.SOCK_STREAM) #initializes socket
sock_d.bind((TCP_IP, DATA_PORT)) #bind ip and port to the socket to get requests
sock_d.listen(1) #starting listening by limit 1 connection


while True:
    conn, addr = sock_f.accept()
    data = conn.recv(buf) #start receiving for buffer sizes 
    if data:
        print ("File name:", data)
        file_name = data.strip()

    f = open(file_name, 'wb')

    conn, addr = sock_d.accept() #accepts requests
    while True:
        data = conn.recv(buf)
        if not data:
            break
        f.write(data)

    print (file_name, 'Finish!')
    f.close()
