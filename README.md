# JsonStrom
This is a pet project in order to intercept JSON data in an Apache Storm data stream.
Mainly this is supposed to be a modular library that is supposed to interact with Seabook (a copy/clone repository of Kafka-Rs).
Working with JSON data, JsonStrom (supposedly Storm-Rs or StormJson/JsonStorm) takes the data, deserializes it while conducting a process of elimination with Bitmap Index; but B-Tree Index might be more appropriate for this project.
I set this up for MIT license so that people can grab it off the repository page and see what ideas they can derive from it.

*I just really like Rust, but in all honesty this would've been better in C or Python because of Apache Thrift, Apache Arrow and the stormpy.py file found in Apache Storm's repository.*
