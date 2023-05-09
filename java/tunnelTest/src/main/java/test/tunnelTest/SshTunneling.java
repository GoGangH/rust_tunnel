package test.tunnelTest;

import java.util.function.Consumer;
import com.jcraft.jsch.JSch;
import com.jcraft.jsch.Session;

public class SshTunneling {

    private static JSch jsch = new JSch();

    private String url = "3.39.85.205";
    private String username = "admin";
    private String password = "rhrkdgus";
    private int port = 22;  //ssh포트
    private int lport = 3000;  //포워딩할 포트
    private int rport = 3306;  //데이터베이스 포트

    private Session session;

    public SshTunneling init(Consumer<Boolean> arg) {
        try {
            session = jsch.getSession(username, url, port); // ssh 접속
            session.setPassword(password); // passwd 입력
            java.util.Properties config = new java.util.Properties();
            config.put("StrictHostKeyChecking", "no"); //접속시 호스트키값을 사용x
            session.setConfig(config); //설정
            session.connect(); //연결
            session.setPortForwardingL(lport, "127.0.0.1", rport); //포워딩 localhost:3000 3.39.85.205:3306
            arg.accept(true);
        } catch (Exception e) {
            arg.accept(false);
        }
        return this;
    }

    public void shutdown() throws Exception {
        if (session != null && session.isConnected()) {
            session.disconnect();
        }
    }
}