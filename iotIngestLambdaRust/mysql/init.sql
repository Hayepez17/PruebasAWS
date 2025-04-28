-- MySQL dump 10.13  Distrib 8.0.19, for Win64 (x86_64)
--
-- Host: localhost    Database: db-teg-test1
-- ------------------------------------------------------
-- Server version	8.0.39

-- /*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
-- /*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
-- /*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
-- /*!50503 SET NAMES utf8mb4 */;
-- /*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
-- /*!40103 SET TIME_ZONE='+00:00' */;
-- /*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
-- /*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
-- /*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
-- /*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `sensors`
--

DROP TABLE IF EXISTS `sensors`;
-- /*!40101 SET @saved_cs_sensor     = @@character_set_sensors */;
-- /*!50503 SET character_set_sensor = utf8mb4 */;
CREATE TABLE `sensors` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `serial` varchar(20) COLLATE utf8mb4_general_ci NOT NULL,
  `description` varchar(250) COLLATE utf8mb4_general_ci NOT NULL,
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_sensor = @saved_cs_sensor */;

--
-- Dumping data for table `sensors`
--

LOCK TABLES `sensors` WRITE;
-- /*!40000 ALTER TABLE `sensors` DISABLE KEYS */;
INSERT INTO `sensors` VALUES 
(1,'ds18b20','FFFFFFF1','','2024-10-13 14:24:59'),
(2,'ds18b20','FFFFFFF2','','2024-10-13 14:24:59'),
(3,'ds18b20','FFFFFFF3','','2024-10-13 14:24:59'),
(4,'ds18b20','FFFFFFF4','','2024-10-13 14:24:59'),
(5,'ds18b20','FFFFFFF5','','2024-10-13 14:24:59'),
(6,'ds18b20','FFFFFFF6','','2024-10-13 14:24:59'),
(7,'slave','0103000A','DHT22 relative humidity sensor','2024-10-13 14:24:59'),
(8,'slave','010301F4','DHT22 relative humidity sensor','2024-10-13 14:24:59');
-- /*!40000 ALTER TABLE `sensors` ENABLE KEYS */;
UNLOCK TABLES;

DROP TABLE IF EXISTS `clients`;
-- /*!40101 SET @saved_cs_client     = @@character_set_client */;
-- /*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `clients` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `telegram_group` varchar(100) COLLATE utf8mb4_general_ci NOT NULL,
  `status` tinyint NOT NULL DEFAULT '1',
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `clients`
--

LOCK TABLES `clients` WRITE;
-- /*!40000 ALTER TABLE `clients` DISABLE KEYS */;
INSERT INTO `clients` VALUES (1,'Soportic','',1,'2024-10-13 14:24:59'),(2,'EIE UCV','',1,'2024-10-13 14:24:59'),(3,'Iambox','',1,'2024-10-13 14:24:59');
-- /*!40000 ALTER TABLE `clients` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `devices`
--

DROP TABLE IF EXISTS `devices`;
-- /*!40101 SET @saved_cs_client     = @@character_set_client */;
-- /*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `devices` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `client_id` bigint NOT NULL,
  `mac_address` varchar(20) COLLATE utf8mb4_general_ci NOT NULL,
  `model` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `status` tinyint NOT NULL DEFAULT '1',
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `devices_ibfk_1` (`client_id`),
  CONSTRAINT `devices_ibfk_1` FOREIGN KEY (`client_id`) REFERENCES `clients` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `devices`
--

LOCK TABLES `devices` WRITE;
-- /*!40000 ALTER TABLE `devices` DISABLE KEYS */;
INSERT INTO `devices` VALUES (1,3,'78:21:84:9C:A9:AC','ESP32',1,'2024-10-13 15:00:05'),(2,3,'3C:61:05:0D:1C:08','ESP32',1,'2024-10-13 15:00:05');
-- /*!40000 ALTER TABLE `devices` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `locations`
--

DROP TABLE IF EXISTS `locations`;
-- /*!40101 SET @saved_cs_client     = @@character_set_client */;
-- /*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `locations` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `client_id` bigint NOT NULL,
  `name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `address` text COLLATE utf8mb4_general_ci NOT NULL,
  `status` tinyint NOT NULL DEFAULT '1',
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `locations_ibfk_1` (`client_id`),
  CONSTRAINT `locations_ibfk_1` FOREIGN KEY (`client_id`) REFERENCES `clients` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `locations`
--

LOCK TABLES `locations` WRITE;
-- /*!40000 ALTER TABLE `locations` DISABLE KEYS */;
INSERT INTO `locations` VALUES (1,3,'RACK1','',1,'2024-10-13 14:55:43'),(2,3,'RACK2','',1,'2024-10-13 14:55:43'),(3,3,'PASILLO FRIO 1','',1,'2024-10-13 14:55:43'),(4,3,'PASILLO FRIO 2','',1,'2024-10-13 14:55:43'),(5,3,'CUARTO SERVIDOR','',1,'2024-10-13 14:55:43'),(6,3,'CUARTO DE DATOS','',1,'2024-10-13 14:55:43');
-- /*!40000 ALTER TABLE `locations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `logger`
--

DROP TABLE IF EXISTS `logger`;
-- /*!40101 SET @saved_cs_client     = @@character_set_client */;
-- /*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `logger` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `device_location_id` bigint NOT NULL,
  `client_name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `location_name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `alarm_level` tinyint NOT NULL DEFAULT '1',
  `msg` text COLLATE utf8mb4_general_ci NOT NULL,
  `recognized` tinyint NOT NULL DEFAULT '0',
  `last_update` datetime NOT NULL,
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `logger`
--

LOCK TABLES `logger` WRITE;
-- /*!40000 ALTER TABLE `logger` DISABLE KEYS */;
-- /*!40000 ALTER TABLE `logger` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `metrics`
--

DROP TABLE IF EXISTS `metrics`;
-- /*!40101 SET @saved_cs_client     = @@character_set_client */;
-- /*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `metrics` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `device_location_id` bigint NOT NULL,
  `client_name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `location_name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `model_mac` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `sensor_name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `variable_name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `unit` varchar(10) COLLATE utf8mb4_general_ci NOT NULL,
  `state` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `ip` varchar(20) COLLATE utf8mb4_general_ci NOT NULL,
  `value` float NOT NULL,
  `timestamp` datetime NOT NULL,
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
-- /*!40101 SET @saved_cs_client     = @@character_set_client */;
-- /*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `client_id` bigint NOT NULL,
  `name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `email` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `phone` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `enable_notifications` tinyint NOT NULL DEFAULT '0',
  `status` tinyint NOT NULL DEFAULT '1',
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `users_ibfk_1` (`client_id`),
  CONSTRAINT `users_ibfk_1` FOREIGN KEY (`client_id`) REFERENCES `clients` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
-- /*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES (1,1,'Hector Alejandro Yepez','hayepez17@gmail.com','+584124568563',2,1,'2024-10-13 14:41:34'),(2,1,'Hector Ramon Yepez','soportichy@gmail.com','+584123137928',0,1,'2024-10-13 14:41:34'),(3,1,'Victor Jose Yepez','','',0,1,'2024-10-13 14:41:34'),(4,2,'Rafael Rivero','','',0,1,'2024-10-13 14:41:34'),(5,2,'Alejandro Herrera','','',0,1,'2024-10-13 14:41:34'),(6,3,'Jackeline Morffe','jmorffe@gmail.com','+584143986352',0,1,'2024-10-13 14:41:34');
-- /*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `variables`
--

DROP TABLE IF EXISTS `variables`;
-- /*!40101 SET @saved_cs_client     = @@character_set_client */;
-- /*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `variables` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `name` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `unit` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `status` tinyint NOT NULL DEFAULT '1',
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `variables`
--

LOCK TABLES `variables` WRITE;
-- /*!40000 ALTER TABLE `variables` DISABLE KEYS */;
INSERT INTO `variables` VALUES (1,'Temperature','°C',1,'2024-10-13 15:01:43'),(2,'R.Humidity','%RH',1,'2024-10-13 15:03:57'),(3,'CO2','ppm',1,'2024-10-13 15:05:41');
-- /*!40000 ALTER TABLE `variables` ENABLE KEYS */;
UNLOCK TABLES;


--
-- Table structure for table `device_locations`
--

DROP TABLE IF EXISTS `device_locations`;
-- /*!40101 SET @saved_cs_client     = @@character_set_client */;
-- /*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `device_locations` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `device_id` bigint NOT NULL,
  `client_id` bigint NOT NULL,
  `location_id` bigint NOT NULL,
  `variable_id` bigint NOT NULL,
  `sensor_id` bigint NOT NULL,
  `notify_every` int unsigned NOT NULL DEFAULT '15',
  `min` float NOT NULL,
  `max` float NOT NULL,
  `warning` float NOT NULL,
  `critical` float NOT NULL,
  `offset` float(10,8) NOT NULL DEFAULT '0.00000000',
  `calibration_factor` float(10,8) NOT NULL DEFAULT '1.00000000',
  `status` tinyint NOT NULL DEFAULT '1',
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `dl_ibfk_1` (`device_id`),
  KEY `dl_ibfk_2` (`client_id`),
  KEY `dl_ibfk_3` (`location_id`),
  KEY `dl_ibfk_4` (`variable_id`),
  KEY `dl_ibfk_5` (`sensor_id`),
  CONSTRAINT `dl_ibfk_1` FOREIGN KEY (`device_id`) REFERENCES `devices` (`id`),
  CONSTRAINT `dl_ibfk_2` FOREIGN KEY (`client_id`) REFERENCES `clients` (`id`),
  CONSTRAINT `dl_ibfk_3` FOREIGN KEY (`location_id`) REFERENCES `locations` (`id`),
  CONSTRAINT `dl_ibfk_4` FOREIGN KEY (`variable_id`) REFERENCES `variables` (`id`),
  CONSTRAINT `dl_ibfk_5` FOREIGN KEY (`sensor_id`) REFERENCES `sensors` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
-- /*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `device_locations`
--

LOCK TABLES `device_locations` WRITE;
-- /*!40000 ALTER TABLE `device_locations` DISABLE KEYS */;
INSERT INTO `device_locations` VALUES 
(1,1,3,5,1,1,15,-30,40,28,30,0.00000000,1.00000000,1,'2024-10-13 15:25:35'),
(2,1,3,5,2,7,15,0,100,80,90,0.00000000,1.00000000,1,'2024-10-13 15:25:35'),
(3,1,3,1,1,2,15,-30,40,28,30,0.00000000,1.00000000,1,'2024-10-13 15:25:35'),
(4,1,3,2,1,3,15,-30,40,28,30,0.00000000,1.00000000,1,'2024-10-13 15:25:35'),
(5,1,3,3,1,4,15,-30,40,28,30,0.00000000,1.00000000,1,'2024-10-13 15:25:35'),
(6,1,3,4,1,5,15,-30,40,28,30,0.00000000,1.00000000,1,'2024-10-13 15:25:35'),
(7,2,3,6,1,6,15,-30,40,28,30,0.00000000,1.00000000,1,'2024-10-13 15:25:35'),
(8,2,3,6,2,8,15,0,100,80,90,0.00000000,1.00000000,1,'2024-10-13 15:25:35');
-- /*!40000 ALTER TABLE `device_locations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Dumping routines for database 'db-teg-test1'
--
-- /*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

-- /*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
-- /*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
-- /*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
-- /*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
-- /*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
-- /*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
-- /*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-10-13 16:09:39
