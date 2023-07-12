#!/usr/bin/env python3

import asyncio
import websockets


def main():
    
    start_server = websockets.serve(handler, "127.0.0.1", 8080)
    asyncio.get_event_loop().run_until_complete(start_server)
    print("server started")

    try:
        asyncio.get_event_loop().run_forever()
    except KeyboardInterrupt:
        print(" - interrupted")
    
async def handler(websocket, path):
        
    print("connected")
    
    try:
        websocket.send_value = 0
        websocket.send_counter = 10
        
        while True:            
            await handler_send(websocket)
            await handler_recv(websocket)
            
    except websockets.exceptions.ConnectionClosedOK:
        print("connection lost")

async def handler_send(websocket):
                                        
    websocket.send_counter -= 1    
    if websocket.send_counter > 0:
        return
    websocket.send_counter = 20   
             
    message = "server->client " + str(websocket.send_value)
    await websocket.send(message)
    websocket.send_value += 1

async def handler_recv(websocket):
                    
    try:
        message = await asyncio.wait_for(websocket.recv(), timeout=0.01)
    except asyncio.exceptions.TimeoutError:
        message = None        

    if message is not None:
        print("received:", message)


if __name__ == "__main__":
    main()    
