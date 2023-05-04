import com.jcraft.jsch.*;

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
        session.setUserInfo(lui);
        session.connect();
        session.setPortForwardingL(tunnelLocalPort, tunnelRemoteHost, tunnelRemotePort);
        System.out.println("Connectd");
    }

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
}