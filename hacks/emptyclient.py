#!/usr/bin/env python3

import socket
import sys
import threading


	
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(("localhost", 9232))


input()
