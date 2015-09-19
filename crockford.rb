require 'set'
C = {}
V = Set.new

def add(str, val)
    raise if V.include?(val)
    V << val
    str.each_char {|c|
        C[c.ord] = val
    }
end

def gen_rust
    puts %{const ALPHABET_INV: [i8; 256] = [}
    a = (0..255).map {|c| (C[c] || -1).to_s.rjust(2) + " /* " + c.to_s.rjust(3) + " */"}
    puts(a.each_slice(8).map{|s|
        "    " + s.join(", ")
    }.join(",\n"))
    puts %{];}
end

def gen_rust2
    first = nil 
    last = nil 
    0.upto(255) do |i|
        if C[i]
            first = i
            break
        end
    end
    255.downto(0) do |i|
        if C[i]
            last = i
            break
        end
    end

    #p first, last

    len = last - first + 1
    
    puts %{const ALPHABET_INV_FIRST: usize = #{first};}
    puts %{const ALPHABET_INV_LAST:  usize = #{last};}
    puts %{const ALPHABET_INV: [i8; #{len}] = [}
    a = (first..last).map {|c| (C[c] || -1).to_s.rjust(2) + " /* " + c.chr.to_s + " */"}
    puts(a.each_slice(8).map{|s|
        "    " + s.join(", ")
    }.join(",\n"))
    puts %{];}
end


add('0Oo', 0)
add('1IiLl', 1)
(2..9).each {|i| add(i.to_s, i)}
add('Aa', 10)
add('Bb', 11)
add('Cc', 12)
add('Dd', 13)
add('Ee', 14)
add('Ff', 15)
add('Gg', 16)
add('Hh', 17)
add('Jj', 18)
add('Kk', 19)
add('Mm', 20)
add('Nn', 21)
add('Pp', 22)
add('Qq', 23)
add('Rr', 24)
add('Ss', 25)
add('Tt', 26)
add('Vv', 27)
add('Ww', 28)
add('Xx', 29)
add('Yy', 30)
add('Zz', 31)

#gen_rust
gen_rust2
