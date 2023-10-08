# FreeDB

** FreeDB is a NatherSQL Databasea....**

## Specification

* 1. CREATE DATABASE *
DB create NameOfMyDatabase

* 2. CREATE TABLES *
CREATE TABLE NameOfMyTable FROM NameOfMyDatabase [ Id String(50), Name String(255), Age Number ];

* 3. INSERT INTO TABLE *
INSERT INTO NameOfMyTable (Id, Name) VALUES ('1', 'Ben Frank')

* 4. SELECT FROM TABLE *
SELECT Id, Name, Age FROM NameOfMyTable
SELECT Id, Name, Age FROM NameOfMyTable WHERE Age > 500