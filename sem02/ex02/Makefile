#diretivas de execução

SRC = ./src
DEMO = ./demo
BIN = ./bin
INCLUDE = ./include
OBJ = ./obj

#flag/target 
all: lib exe #dependências
#gcc -c ./src/test.c -I ./include -o ./obj/test.o
#gcc -c $(SRC)/test.c -I ./include -o ./obj/test.o
#gcc ./demo/app.c ./obj/*.o -I ./include -o ./bin/app
	echo "Makefile is working"

run:
	./bin/app
	echo "Running program"

lib: test.o
#	gcc -c $(SRC)/test.c -I ./include -o ./obj/test.o

exe:
	gcc ./demo/app.c ./obj/*.o -I ./include -o ./bin/app


#template de linkagem
#   $<  ----> acesso ao primeiro item de dependência
#   @ valor do target/flag
%.o: $(SRC)/%.c $(INCLUDE)/%.h
	gcc -c $< -I $(INCLUDE) -o $(OBJ)/$@
#   gcc -c $(SRC)/%.c -I %(INCLUDE) -o $(OBJ)/$teste.o

hello: #make hello indica flag de execução
	echo "Diretiva hello!"
	tree .

clean:
	rm -rf ./bin/* ./obj/*