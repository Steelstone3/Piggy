using System.Collections.Generic;

public class JobFactory : IJobFactory
{
    public List<Job> Create()
    {
        return new List<Job>()
        {
            new Job
            {
                Name = "dotnet-run",
                SubFolder = "",
                Command = "dotnet",
                Arguments = new List<string>() { "run" }
            },
            new Job
            {
                Name = "dotnet-test",
                SubFolder = "ProjectTests",
                Command = "dotnet",
                Arguments = new List<string>() { "test" }
            }
        };
    }
}

public interface IJobFactory
{
    List<Job> Create();
}