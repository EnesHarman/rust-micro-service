# Project Overview

This project is composed of three main modules:

## 1. Shared Module
The Shared module contains shared structs and data types utilized by both the API and Event modules. This module ensures consistency and reusability across the project.

### Key Features:
- Centralized definition of shared data models.
- Eliminates redundancy by providing common types.

---

## 2. Event Module
The Event module is responsible for storing consumed events in MongoDB and handling Kafka consumer operations. It connects to the event topic in Kafka to consume messages and process them for storage.

### Key Features:
- Kafka consumer integration for real-time event processing.
- MongoDB support for event persistence.

---

## 3. API Module
The API module provides RESTful services to create events. It exposes endpoints that allow clients to interact with the system and initiate event creation.

### Key Features:
- User-friendly RESTful API for event creation.
- Input validation and error handling.

---


