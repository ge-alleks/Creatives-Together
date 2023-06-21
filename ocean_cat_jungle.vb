Public Class CreativesTogether
	Public Shared Sub Main()
		
		Dim title As String = "Creatives Together"
	
		' Print the title of the program 
		Console.WriteLine(title)
		
		' Initialize the variables 
		Dim userName As String = ""
		Dim userAge As Integer = 0
		Dim numOfProjects As Integer = 0 
		Dim userType As String = ""
		Dim userPrefs As String = ""
		
		' Read the input from the user 
		Console.Write("Please enter your name: ")
		userName = Console.ReadLine()
		
		Console.Write("Please enter your age: ")
		userAge = Console.ReadLine()
		
		Console.Write("Please enter the number of projects you have completed: ")
		numOfProjects = Console.ReadLine()
		
		Console.Write("Please enter the type of creative you are (ex. photographer, artist, filmmaker, etc.): ")
		userType = Console.ReadLine()
		
		Console.Write("Please enter your creative preferences (ex. style, technique, etc.): ")
		userPrefs = Console.ReadLine()
		
		' Print out the user's information 
		Console.WriteLine("Name: {0}", userName)
		Console.WriteLine("Age: {0}", userAge)
		Console.WriteLine("Projects Completed: {0}", numOfProjects)
		Console.WriteLine("Type of Creative: {0}", userType)
		Console.WriteLine("Creative Preferences: {0}", userPrefs)
		
		' Read input from user to accept or decline 
		Dim userInput As String = ""
		
		Console.WriteLine("Do you accept the terms for joining Creatives Together? (Y or N): ")
		userInput = Console.ReadLine()
		
		' Print out a message depending on user input 
		If userInput = "Y" Then
			Console.WriteLine("Welcome to Creatives Together! We look forward to making amazing things together.")
		Else
			Console.WriteLine("We are sorry to hear you will not be joining us. We hope to see you soon!")
		End If
	
	End Sub
End Class