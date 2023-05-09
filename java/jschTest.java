import com.jcraft.jsch.*; //라이브러리 추가

public class Tunnel {
    public static void main(String[] args) {
        Tunnel t = new Tunnel();
        try {
            t.go();
        } catch (Exception ex) {
            ex.printStackTrace();
        }
    }

    public void go() throws Exception {
        String host = "XXX.XXX.XXX.XXX";
        String user = "user";
        String password = "password";
        int port = 22;

        int tunnelLocalPort = 9080;
        String tunnelRemoteHost = "YYY.YYY.YYY.YYY";
        int tunnelRemotePort = 80;

        JSch jsch = new JSch();
        Session session = jsch.getSession(user, host, port);
        session.setPassword(password);
        localUserInfo lui = new localUserInfo();
        session.setUserInfo(lui); // ssh 연결 설정
        session.connect(); // ssh 연결
        session.setPortForwardingL(tunnelLocalPort, tunnelRemoteHost, tunnelRemotePort);
        System.out.println("Connectd");
    }

    class localUserInfo implements UserInfo {
        String passwd;

        public String getPassword() {
            return passwd;
            // 사용자가 입력한 암호를 반환합니다.
        }

        public boolean promptYesNo(String str) {
            return true;
            // 예-아니오-질문에 대답하라는 메시지를 사용자에게 표시합니다.
        }

        public String getPassphrase() {
            return null;// 사용자가 입력한 암호를 반환합니다.
        }

        public boolean promptPassphrase(String message) {
            return true;
            // 사용자에게 공개 키의 암호를 묻는 메시지를 표시합니다.
        }

        public boolean promptPassword(String message) {
            return true;
            //원격 서버의 인증에 사용되는 암호를 사용자에게 묻습니다.
        }
