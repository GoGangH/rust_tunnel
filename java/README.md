# JSch

> JSch는 reverse tunnel을 구현할수 있게 해주는 java라이브러리이다.

- JSch를 이용하여 reverse tunneling 구현

## maven project 적용방법

```java
//maven
<dependency>
    <groupId>com.jcraft</groupId>
    <artifactId>jsch</artifactId>
    <version>0.1.54</version>
</dependency>
```

## 설정해야할 것

![image](http://www.beanizer.org/img/tunneling.png)

- 설정할 값

```java
//filewall 암호 및 주소
String host = "XXX.XXX.XXX.XXX";
String user = "user";
String password = "password";
int port = 22;

//현재 접속 포트
int tunnelLocalPort = 9080;

//접속할 대상
String tunnelRemoteHost = "YYY.YYY.YYY.YYY";
int tunnelRemotePort = 80;
```

- ssh 연결

```java
JSch jsch = new JSch(); //jsch 객체 생성
Session session = jsch.getSession(user, host, port); // ssh 연결
session.setPassword(password); // 암호 설정
```

-

```java
class localUserInfo implements UserInfo {
    String passwd;

    public String getPassword() {
        return passwd;
    }

    public boolean promptYesNo(String str) {
        return true;
    }

    public String getPassphrase() {
        return null;
    }

    public boolean promptPassphrase(String message) {
        return true;
    }

    public boolean promptPassword(String message) {
        return true;
    }

    public void showMessage(String message) {
    }
}
```
