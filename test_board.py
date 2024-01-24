from csv import reader

board_length = 0
board = ""

with open("board.txt") as f:
    reading = reader(f)
    for line in reading:
        if not line:
            break
        sum = 0
        board += "|"
        for char in line[0]:
            if char.isdigit():
                sum += int(char)
                board += "  " * int(char)
            elif char == "_":
                sum += 1
                board += "_ "
            else: # "|"
                board = board[:-1] + "|"
        board = board[:-1] + "|\n"
        if not board_length: # initialization
            board_length = sum
        elif sum != board_length:
            print("uh oh!", line, sum, board_length)
        
top =  " " + "_ " * board_length + "\n"
bottom =  " " + "‾ " * board_length
board = top + board + bottom
print(board)