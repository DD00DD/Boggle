defmodule Boggle do

  @moduledoc """
    Add your boggle function below. You may add additional helper functions if you desire. 
    Test your code by running 'mix test' from the tester_ex_simple directory.
  """

  def boggle(board, words) do
    row = tuple_size(elem(board, 0))
    col = tuple_size(board)

    b = { {"e", "a"}, 
          {"s", "t"} }
    w = ["seat", "sat", "at", "a", "dude", "pee"]

    Enum.reduce(words, %{}, fn word, dict ->
      if String.length(word) <= (row*col) do
        tmp = check(board, word, row, col, 0, 0, nil)
        
        if tmp == nil do
          dict
        else
          dict =  Map.put(dict, word, tmp)
        end
      else 
        dict
      end
    end)
  end

  #prints out found word when it is not nil or when i is done iterating
  def check(board, word, row, col, i, j, found) when i >= row or found != nil do
    found
  end
  
  #reset j and increment i
  def check(board, word, row, col, i, j, found) when j >= col do
    check(board, word, row, col, i+1, 0, found)
  end

  def check(board, word, row, col, i, j, found) when i < row and j < col do
    cond do 
      elem(elem(board, i), j) != String.at(word, 0) -> check(board, word, row, col, i, j+1, found)
      true -> check(board, word, row, col, i, j+1, match(board, word, String.length(word), row, col, i, j, 1, []))
    end
  end
  
  
  def match(board, word, strSize, row, col, i, j, length, visit) when length <= strSize do
    cond do
      #check if out of bounds
      (i < 0) or (i > (row - 1)) -> nil
      (j < 0) or (j > (col - 1)) -> nil

      #compare character on board and in word for a match
      elem(elem(board, i), j) != String.at(word, (length-1)) -> nil

      length > String.length(word) -> nil 
  
      #check if visit already contains the coordinates
      Enum.member?(visit, {i, j}) -> nil

      length == strSize -> Enum.reverse([{i, j} | visit])
      #maybe check if found word is not in dict yet

      true -> 
        #prepend new coordinate to previous visit 
        a =(match(board, word, strSize, row, col, i-1, j, length+1, [{i, j} | visit]) ||
            match(board, word, strSize, row, col, i+1, j, length+1, [{i, j} | visit]) ||
            match(board, word, strSize, row, col, i, j-1, length+1, [{i, j} | visit]) ||
            match(board, word, strSize, row, col, i, j+1, length+1, [{i, j} | visit]) ||
            match(board, word, strSize, row, col, i-1, j+1, length+1, [{i, j} | visit]) ||
            match(board, word, strSize, row, col, i-1, j-1, length+1, [{i, j} | visit]) ||
            match(board, word, strSize, row, col, i+1, j-1, length+1, [{i, j} | visit]) ||
            match(board, word, strSize, row, col, i+1, j+1, length+1, [{i, j} | visit]))
    end
  end
end