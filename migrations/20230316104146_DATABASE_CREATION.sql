CREATE TABLE ROOM(
    roomId SERIAL PRIMARY KEY,
    roomName VARCHAR(255) NOT NULL
);

CREATE TABLE SENSOR(
    sensorMacAdresse MACADDR PRIMARY KEY,
    sensorName VARCHAR(255) NOT NULL
);

CREATE TABLE LOCATED_IN(
    locatedInRoomId INTEGER NOT NULL,
    locatedInSensorMacAdresse MACADDR NOT NULL,
    CONSTRAINT PK_LOCATED_IN PRIMARY KEY(locatedInRoomId, locatedInSensorMacAdresse),
    CONSTRAINT FK_LOCATED_IN_ROOM FOREIGN KEY(locatedInRoomId) REFERENCES ROOM(roomId),
    CONSTRAINT FK_LOCATED_IN_SENSOR FOREIGN KEY(locatedInSensorMacAdresse) REFERENCES SENSOR(sensorMacAdresse)
);

CREATE TABLE RECORD(
    recordRoomId INTEGER NOT NULL,
    recordDateTime TIMESTAMP NOT NULL,
    recordTemperature REAL NOT NULL,
    recordHumidity REAL NOT NULL,
    CONSTRAINT PK_RECORD PRIMARY KEY(recordRoomId, recordDateTime),
    CONSTRAINT FK_RECORD_ROOM FOREIGN KEY(recordRoomId) REFERENCES ROOM(roomId)
);