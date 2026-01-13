import socket
import threading

RELAY_IP = ('127.0.0.1', 4001) # what players connect to
BUFFER_SIZE = 4096

# Servers
SERVERS = {
  "server1": ("127.0.0.1", 25565),
}

def read_varint(data, index=0):
  num_read = 0
  result = 0
  while True:
    byte = data[index]
    index += 1
    value = byte & 0b01111111
    result |= value << (7 * num_read)
    print(f"result: {bin(result)} {index}")
    num_read += 1

    if not (byte & 0b10000000):
      break
    if num_read > 5:
      raise ValueError("VarInt is too big")
  return result, index

def parse_handshake(data, idx):

  # Server address (length-prefixed string)
  addr_length, idx = read_varint(data, idx)
  server_address = data[idx:idx+addr_length].decode("utf-8")
  idx += addr_length

  # Server port (unsigned short)
  server_port = int.from_bytes(data[idx:idx+2], byteorder="big")
  idx += 2

  # Next state (VarInt)
  next_state, idx = read_varint(data, idx)

  return server_address, server_port

def forward(src, dst):
  while True:
    data = src.recv(BUFFER_SIZE)
    if data:
      print(src, dst, data);
      dst.sendall(data)

def get_server_sock(server_address):
  server_name = server_address.split(".")[0]
  if server_name not in SERVERS:
    print("Invalid server name:", server_name)
    return None

  server_ip = SERVERS[server_name]
  server_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  server_sock.connect(server_ip)
  return server_sock

def handle_client(client_sock):
  server_sock = None

  while True:
    try:
      data = client_sock.recv(BUFFER_SIZE)
      if not data:
        continue

      idx = 0
      packet_length, idx = read_varint(data, idx)
      packet_id, idx = read_varint(data, idx)
      protocol_version, idx = read_varint(data, idx)

      # TODO: Check for status from back of data

      print(protocol_version)
      print(packet_length, packet_id, data)

      if packet_id == 0x00 and packet_length != 1 and protocol_version == 773:
        try:
          server_address, server_port = parse_handshake(data, idx)
          print("Client wants to connect to:", server_address, server_port)
          server_sock = get_server_sock(server_address)
          if server_sock:
            threading.Thread(target=forward, args=(server_sock, client_sock)).start()
          print(server_sock)
        except:
          pass

        if server_sock:
          server_sock.sendall(data)

      else:
        if server_sock:
          server_sock.sendall(data)
    except Exception as e:
      print(e)

def main():
  s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  s.bind(RELAY_IP)
  s.listen(5)
  print(f"Listening on {RELAY_IP}")
  while True:
    print("Player connected")
    client_sock, _ = s.accept()
    threading.Thread(target=handle_client, args=(client_sock,)).start()

if __name__ == "__main__":
  main()

