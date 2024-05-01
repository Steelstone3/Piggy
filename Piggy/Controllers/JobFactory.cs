using System.Collections.Generic;

public class JobFactory : IJobFactory
{
    public List<Job> Create()
    {
        return new List<Job>()
        {
            new Job
            {
                Name = "dotnet-test",
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