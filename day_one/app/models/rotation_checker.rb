class RotationChecker < ApplicationRecord
    def self.leftPointingZero
        dial = 50;
        password = 0;  #0 Counter
        rotations = File.read(Rails.root.join("lib", 'inputs', 'rotations.txt'))


        for rotation in rotations.split("\n") do
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
end