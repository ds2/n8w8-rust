@0xd172d3627e622346;

struct Date {
  year @0 :Int16;
  month @1 :UInt8;
  day @2 :UInt8;
}

struct Map(K, V) {
  entries @0 :List(Entry);
  struct Entry {
    key @0 :K;
    value @1 :V;
  }
}

struct N8w8TestResultValues {
    startTime @0 :UInt64;
    stopTime @1 :UInt64;
    successful @2 :Bool;
    errorMessage @3 :Text;
}

struct AuthBasicCredentials {
    username @0 :Text;
    password @1 :Text;
}

struct NoCredentials {
}

struct AuthCredentials {
    union {
        empty @0 :NoCredentials;
        basic @1 :AuthBasicCredentials;
    }
}

struct HttpCheckParams {
    url @0 :Text;
    connectT0 @1 :UInt16;
    readT0 @2 :UInt16;
    credentials @3 :AuthCredentials;
}

interface N8w8Test {
    setParams @0 (paramsMap :Map(Text, Text));
    runTest @1 ();
    getResult @2 () -> (result :N8w8TestResultValues);
}

struct Account {
    id @0 :UInt64;
    displayName @1 :Text;
    email @2 :Text;
    active @3 :Bool;
    created @4 :UInt64;
    modified @5 :UInt64;
}
