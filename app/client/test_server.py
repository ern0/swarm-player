#!/usr/bin/env python3

import asyncio
import websockets


def main():
    
    start_server = websockets.serve(handler, "127.0.0.1", 8080)
    asyncio.get_event_loop().run_until_complete(start_server)
    asyncio.get_event_loop().run_forever()
    
async def handler(websocket, path):
    
    send_value = 0
    send_counter = 10
        
    while True:
            
        send_counter -= 1    
        if send_counter == 0:
            send_counter = 20            
            await websocket.send(str(send_value))
            send_value += 1
            
        try:
            message = await asyncio.wait_for(websocket.recv(), timeout=0.01)
        except asyncio.exceptions.TimeoutError:
            message = None        
        if message is not None:
            print(message)

if __name__ == "__main__":
    main()    
