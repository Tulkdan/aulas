package presentation;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Random;

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

}
