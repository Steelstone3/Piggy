using System.Collections.Generic;
using Xunit;

public class JobFactoryShould
{
    [Fact]
    public void CreateJobs()
    {
        // Given
        List<Job> expectedDefaultJobs = new()
        {
            new Job 
            {
                Name = "dotnet-test",
                Command = "dotnet",
                Arguments = new List<string>() { "test" }
            }
        };
        IJobFactory jobFactory = new JobFactory();

        // When
        List<Job> defaultJobs = jobFactory.Create();

        // Then
        Assert.Equivalent(expectedDefaultJobs, defaultJobs);
    }
}