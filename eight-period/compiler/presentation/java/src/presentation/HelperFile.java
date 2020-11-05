package presentation;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Random;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class HelperFile {

  public ArrayList<String> data = new ArrayList<String>();

  public void open(String filename) throws Exception {
    String fullPath = "/home/tulkdan/Documents/aulas/eight-period/compiler/presentation/";
    File file = new File(fullPath + filename);

    BufferedReader br = new BufferedReader(new FileReader(file));

    String st;
    while ((st = br.readLine()) != null) {
      data.add(st);
    }
  }

  public String getRandomItem() {
      Random randomGenerator = new Random();
      int index = randomGenerator.nextInt(data.size());
      String item = data.get(index);
      return item;
  }

    /**
   * Use Files class from Java 1.7 to write files, internally uses OutputStream
   * @param data
   */
  public static void write(String data) {
      try {
          Files.write(Paths.get("output.csv"), data.getBytes());
      } catch (IOException e) {
          e.printStackTrace();
      }
  }

}
