import socket
import sys

TCP_IP = "127.0.0.1" #defining server address
FILE_PORT = 5005 #defining file port to transfer
DATA_PORT = 5006 #defining data port to transfer
buf = 1024 #defining the header size which will be send
file_name = sys.argv[1] #get argument from command line


try:
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM) #initializes socket
    sock.connect((TCP_IP, FILE_PORT)) #connect to the server by the socket defined
    sock.send(file_name) #send file data through the socket
    sock.close() #ends the communication

    print (f"Sending {file_name} ...")
    f = open(file_name, "rb")
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM) #initializes another socket
    sock.connect((TCP_IP, DATA_PORT)) #connect to the server by the socket defined
    data = f.read() #read data from the file
    sock.send(data) #send back to the server through the socket

#one port is to send the file names and another to send the load from the file

finally:
    sock.close()
    f.close()
