package failure.helloworld.desktop;

import com.badlogic.gdx.backends.lwjgl3.Lwjgl3Application;
import com.badlogic.gdx.backends.lwjgl3.Lwjgl3ApplicationConfiguration;
import failure.helloworld.HelloWorld;

public class DesktopLauncher {
	public static void main (String[] arg) {
		Lwjgl3ApplicationConfiguration config = new Lwjgl3ApplicationConfiguration();
        config.setTitle("HelloWorld");
        config.setWindowedMode(800, 480);
		new Lwjgl3Application(new HelloWorld(), config);
	}
}
