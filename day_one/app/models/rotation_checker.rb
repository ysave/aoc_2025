class RotationChecker < ApplicationRecord
    def self.part_one
        dial = 50;
        password = 0;  #0 Counter
        rotations = File.read(Rails.root.join("lib", 'inputs', 'rotations.txt')).split("\n")

        rotations.each do |rotation|
            password += 1 if dial == 0
            if rotation.include?("R")
                dial += rotation.split("R").second.to_i
                dial = dial % 100 if dial >= 100
            else
                dial -= rotation.split("L").second.to_i
                dial = dial % 100 if dial < 0
            end
        end
        password
    end

    def self.part_two
        dial = 50
        password = 0  # Wrap counter
        rotations = File.read(Rails.root.join("lib", 'inputs', 'rotations.txt')).split("\n")

        rotations.each do |rotation|
            if rotation.include?("R")
                value = rotation.split("R").second.to_i
                distance_to_boundary = (100 - dial) % 100
                distance_to_boundary = 100 if distance_to_boundary == 0
                
                if value >= distance_to_boundary
                    password += 1 + (value - distance_to_boundary) / 100
                end
                
                dial += value
                dial = dial % 100 if dial >= 100
            else
                value = rotation.split("L").second.to_i
                distance_to_boundary = dial % 100
                distance_to_boundary = 100 if distance_to_boundary == 0
                
                if value >= distance_to_boundary
                    password += 1 + (value - distance_to_boundary) / 100
                end
                
                dial -= value
                dial = dial % 100 if dial < 0
            end
        end
        
        password
    end
end