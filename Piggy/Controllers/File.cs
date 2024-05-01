using Newtonsoft.Json;

public class File : IFile
{
    public bool IsExistingFile(string filePath)
    {
        return System.IO.File.Exists(filePath);
    }

    public void CreateDefaultPiggyConfigurationFile()
    {
        Piggy piggy = new(){
            Jobs = new JobFactory().Create()
        };

        string json = JsonConvert.SerializeObject(piggy, Formatting.Indented);

        System.IO.File.WriteAllText("piggy.json", json);
    }

    public Piggy ReadPiggyConfigurationFile(string filePath)
    {
        string json = System.IO.File.ReadAllText(filePath);

        return JsonConvert.DeserializeObject<Piggy>(json);
    }
}

public interface IFile
{
    bool IsExistingFile(string filePath);
    void CreateDefaultPiggyConfigurationFile();
    Piggy ReadPiggyConfigurationFile(string filePath);
}
