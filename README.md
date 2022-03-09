# goomble-examples
Several implementations of the key logic of the Goomble problem set from Saltzer &amp; Kaashoek's "Principles of Computer System Design"

## Java version

To do anything with the Java version, make sure you first go into the
`Java` directory:

```
cd Java
```

### Building

To build and run the java version, one of the easier ways is to use

```text
./gradlew jar
```

to build a standalone JAR file which you can then run with or
without locks:

```text
java -jar app/build/libs/app.jar
java -jar app/build/libs/app.jar --lock
```

If you precede either of this with `time` then the system will report
some timing information:

```text
time java -jar app/build/libs/app.jar
time java -jar app/build/libs/app.jar --lock
```
