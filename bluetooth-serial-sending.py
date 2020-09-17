import serial;
import sys
import os
import time
import paho.mqtt.client as mqtt
import threading
import json as JSON
mqttc = {}
container = []
send_container = []
strBroker = "127.0.0.1"
port = 1883
ser = serial.Serial()
def configureSerial(portName, rate):
	ser.port = portName
	ser.baudrate = rate
	ser.timeout = 0.2
	ser.stopbits = 1
	ser.bytesize = 8
	try:
		ser.open()
	except Exception as e:
		print("Error:", e)
		print("exit...")
		return 1
	finally:
		return 0

def recv(serial):
	while True:
		data = serial.read(300)
		if len(data) == 0:
			time.sleep(0.1)	
		else:
			dealData(data)
		time.sleep(0.1)

def genPubString(data):
	json = {
		"token":"",
		"timestamp":"",
		"body": {
			"type":255,
			"len": 33,
			"data":{}
		}
	}
	result = "";
	for d in data:
		if d < 16:
			result = result + "0" + hex(d)
		else:
			result = result + hex(d)
	json["body"]["data"] = result.replace("0x","");
	return JSON.dumps(json)

def dealData(data):
	global container;
	global mqttc;
	for d in data[:]:
		container.append(d)
	i = 0
	if (len(container)>=7):
		while (i<len(container)):
			if(container[i]==105):
				if (i+2)>=len(container):
					break
				else :
					if (i + container[i+1]+container[i+2]*256)<=len(container):
						data_ = container[i:(i + container[i+1]+container[i+2]*256)]
						if (len(data_)<7):
							i = i + 1;
						else :
							if data_[-1] == 67:
								print(data_)
								if data_[3] == int("82", 16):
									print("fuck you");
									ser.write(bytes([
										int("69",16), int("27", 16), 0, 2, 0, 
										int("aa", 16), int("aa", 16), int("aa", 16), int("aa", 16), int("aa", 16), int("aa", 16),
										 1, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
										 int("4c", 16), int("61", 16), int("43", 16)
									]));
								if data_[3] == 157:
									ser.write(bytes([
										int("69",16), int("0d", 16), 0, int("1d", 16), 
										11, 1, 1, 1, 1, 16, 
										 int("5a", 16), int("dc", 16), int("43", 16)
									]));
								mqttc.publish("comlm/notify/message/rfmanage/rfmanage", genPubString(data_))
								i = i +container[i+1]+container[i+2]*256;
							else : 
								i = i + 1
					else:
						break;
			else :
				i = i + 1
	container = container[i:len(container)]

def dealsend(data):
	global send_container;
	global ser;
	for d in data[:]:
		send_container.append(d)
	i = 0
	if (len(send_container)>=7):
		while (i<len(send_container)):
			if(send_container[i]==105):
				if (i+2)>=len(send_container):
					break
				else :
					if (i + send_container[i+1]+send_container[i+2]*256)<=len(send_container):
						data_ = send_container[i:(i + send_container[i+1]+send_container[i+2]*256)]
						if (len(data_)<7):
							i = i + 1;
						else :
							if data_[-1]==67:
								print(data_);
								ser.write(bytes(data_));
								i = i +send_container[i+1]+send_container[i+2]*256;
							else : 
								i = i + 1
					else:
						break;
			else :
				i = i + 1
	send_container = send_container[i:len(send_container)]
		
def mqtt_init():
	global mqttc;
	mqttc = mqtt.Client(clean_session=True, userdata=None, protocol=mqtt.MQTTv31, transport="tcp")
	mqttc.on_message = on_message
	mqttc.on_connect = on_connect
	mqttc.on_publish = on_publish
	mqttc.on_subscribe = on_subscribe
	mqttc.on_log = on_log
	mqttc.connect(strBroker, port, 30)
	mqttc.subscribe("rfmanage/#", 2);
	mqttc.loop_forever()

def on_connect(mqttc, obj, flags, rc):
	print("OnConnetc, rc: " + str(rc))


def on_publish(mqttc, obj, mid):
	print("OnPublish, mid: " + str(mid))


def on_subscribe(mqttc, obj, mid, granted_qos):
	print("Subscribed: " + str(mid) + " " + str(granted_qos))


def on_log(mqttc, obj, level, string):
	print("Log:" + string)


def on_message(mqttc, obj, msg):
	if msg.topic == "rfmanage/notify/message/comlm/comlm":
		try:
			data = JSON.loads(bytes.decode(msg.payload))
			frame = data["body"]["data"]
			frame_array = []
			i = 0
			while i < len(frame):
				frame_array.append(int(frame[i:i+2], 16));
				i = i + 2
			print(frame_array);
			dealsend(frame_array)
		except Exception as  e:
			print(e)

	print(msg.topic + " " + str(msg.qos) + " " + str(msg.payload))

def main():
	result=configureSerial(sys.argv[1], int(sys.argv[2]))
	recv(ser);

	
if __name__ == '__main__':
	print(sys.argv)
	print(int("82", 16))
	t = threading.Thread(target=mqtt_init);
	t.start();
	if(len(sys.argv)<2):
		print("please cin enough params")
	else :
		main()  # judge cmd Right
